use abyss_vm::vm::core::AbyssVm;

pub fn abyss_alloc(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let requested_size = args[0] as usize;
    let total_required_size = requested_size + 8;

    let mut selected_index = None;
    for (i, &(_, size)) in vm.free_blocks.iter().enumerate() {
        if size >= total_required_size {
            selected_index = Some(i);
            break;
        }
    }

    let alloc_offset = if let Some(i) = selected_index {
        let (offset, block_size) = vm.free_blocks.remove(i);

        let size_bytes = (block_size as u64).to_le_bytes();
        vm.heap[offset..offset + 8].copy_from_slice(&size_bytes);

        offset
    } else {
        let offset = vm.heap.len();
        vm.heap.resize(offset + total_required_size, 0);

        let size_bytes = (total_required_size as u64).to_le_bytes();
        vm.heap[offset..offset + 8].copy_from_slice(&size_bytes);

        offset
    };

    (alloc_offset + 8) as u64
}

pub fn abyss_free(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let ptr = args[0] as usize;

    if ptr < 8 || ptr > vm.heap.len() {
        return 0;
    }

    let header_offset = ptr - 8;

    let mut size_bytes = [0u8; 8];
    size_bytes.copy_from_slice(&vm.heap[header_offset..header_offset + 8]);
    let block_size = u64::from_le_bytes(size_bytes) as usize;

    vm.free_blocks.push((header_offset, block_size));

    for i in ptr..(header_offset + block_size) {
        vm.heap[i] = 0;
    }

    0
}
