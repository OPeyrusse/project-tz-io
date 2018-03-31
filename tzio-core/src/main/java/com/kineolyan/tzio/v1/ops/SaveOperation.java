package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;

class SaveOperation implements Operation {

	private final int slot;

	public SaveOperation(int slot) {
		this.slot = slot;
	}

	@Override
	public Shift execute(final Node node) {
		node.bakValue(this.slot);
		return Shift.NEXT;
	}
}
