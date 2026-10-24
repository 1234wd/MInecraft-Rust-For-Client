use std::sync::{Arc, Mutex};

use crate::sweep_probe::rows;
use minecraft_rust::com::mojang::blaze3d::framegraph::FramePass::FramePass;
use minecraft_rust::com::mojang::blaze3d::resource::ResourceDescriptor::ResourceDescriptor;
use minecraft_rust::com::mojang::blaze3d::resource::ResourceHandle::ResourceHandle;

struct ValueHandle<T>(T);

impl<T> ResourceHandle<T> for ValueHandle<T> {
    fn get(&self) -> &T {
        &self.0
    }
}

struct IntDescriptor;

impl ResourceDescriptor<i32> for IntDescriptor {
    fn allocate(&self) -> i32 {
        19
    }

    fn free(&self, _resource: i32) {}
}

#[derive(Default)]
struct TestPass {
    reads: usize,
    culling_disabled: bool,
    tasks: Vec<Arc<dyn Fn()>>,
}

impl FramePass for TestPass {
    fn creates_internal<T: 'static>(
        &mut self,
        _name: String,
        descriptor: Arc<dyn ResourceDescriptor<T>>,
    ) -> Arc<dyn ResourceHandle<T>> {
        Arc::new(ValueHandle(descriptor.allocate()))
    }

    fn reads<T>(&mut self, _handle: Arc<dyn ResourceHandle<T>>) {
        self.reads += 1;
    }

    fn reads_and_writes<T>(
        &mut self,
        handle: Arc<dyn ResourceHandle<T>>,
    ) -> Arc<dyn ResourceHandle<T>> {
        self.reads += 1;
        handle
    }

    fn requires<P: FramePass>(&mut self, _pass: Arc<P>) {}

    fn disable_culling(&mut self) {
        self.culling_disabled = true;
    }

    fn executes(&mut self, task: Arc<dyn Fn()>) {
        self.tasks.push(task);
    }
}

#[test]
fn frame_pass_trait_keeps_generic_resources_and_deferred_runnable_callable() {
    let mut pass = TestPass::default();
    let handle = pass.creates_internal("color".to_owned(), Arc::new(IntDescriptor));
    assert_eq!(*handle.get(), 19);
    pass.reads(Arc::clone(&handle));
    let same = pass.reads_and_writes(Arc::clone(&handle));
    assert!(Arc::ptr_eq(&handle, &same));
    pass.requires(Arc::new(TestPass::default()));
    pass.disable_culling();

    let executions = Arc::new(Mutex::new(0));
    let captured = Arc::clone(&executions);
    pass.executes(Arc::new(move || *captured.lock().unwrap() += 1));
    pass.tasks[0]();
    pass.tasks[0]();

    assert_eq!(pass.reads, 2);
    assert!(pass.culling_disabled);
    assert_eq!(*executions.lock().unwrap(), 2);
}

#[test]
fn frame_pass_declared_java_api_matches_the_26_2_jar() {
    let mut actual: Vec<String> = rows("S26_API")
        .into_iter()
        .filter(|row| row[1] == "FramePass")
        .map(|row| format!("{}\t{}\t{}\t{}", row[1], row[2], row[3], row[4]))
        .collect();
    actual.sort();
    let mut expected = vec![
        "FramePass\tmethod\tcreatesInternal(java.lang.String,com.mojang.blaze3d.resource.ResourceDescriptor<T>)com.mojang.blaze3d.resource.ResourceHandle<T>\tpublic abstract".to_owned(),
        "FramePass\tmethod\tdisableCulling()void\tpublic abstract".to_owned(),
        "FramePass\tmethod\texecutes(java.lang.Runnable)void\tpublic abstract".to_owned(),
        "FramePass\tmethod\treads(com.mojang.blaze3d.resource.ResourceHandle<T>)void\tpublic abstract".to_owned(),
        "FramePass\tmethod\treadsAndWrites(com.mojang.blaze3d.resource.ResourceHandle<T>)com.mojang.blaze3d.resource.ResourceHandle<T>\tpublic abstract".to_owned(),
        "FramePass\tmethod\trequires(com.mojang.blaze3d.framegraph.FramePass)void\tpublic abstract".to_owned(),
    ];
    expected.sort();
    assert_eq!(actual, expected);

    let supers: Vec<(String, String, String)> = rows("S26_SUP")
        .into_iter()
        .filter(|row| row[1] == "FramePass")
        .map(|row| (row[1].to_owned(), row[2].to_owned(), row[3].to_owned()))
        .collect();
    assert_eq!(
        supers,
        [("FramePass".into(), "-".into(), "interface".into())]
    );
}
