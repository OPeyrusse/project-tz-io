package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;

class SwapOperation implements Operation {

	private final int slot;

	public SwapOperation(int slot) {
		this.slot = slot;
	}

	@Override
	public Shift execute(final Node node) {
		node.swapValue(this.slot);
		return Shift.NEXT;
	}
}
