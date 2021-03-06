use futures::executor::block_on;

fn main() {
    // This is the eBPF program, in the form of bytecode instructions.
    let prog = &[ 0xb4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov32 r0, 0 
    0xb4, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // mov32 r1, 2 
    0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // add32 r0, 1 
    0x0c, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // add32 r0, r1 
    0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // exit ];
    ];
    // Instantiate a struct EbpfVmNoData. This is an eBPF VM for programs that 
    // takes no packet data in argument. 
    // The eBPF program is passed to the constructor.
    let vm = rbpf::EbpfVmNoData::new(Some(prog)).unwrap();

    block_on(async move{
        dummy(vm.execute_program().unwrap()).await;
    });
}

async fn dummy(result: u64){
    println!("hello world! Result is {} (should be 0x3)", result);
}