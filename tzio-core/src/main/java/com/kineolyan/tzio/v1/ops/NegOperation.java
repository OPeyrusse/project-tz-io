package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;

class NegOperation implements Operation {

	public static final NegOperation INSTANCE = new NegOperation();

	private NegOperation() {}

	@Override
	public Shift execute(final Node node) {
		node.negate();
		return Shift.NEXT;
	}
}
