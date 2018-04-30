package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;

/**
 * Operation negating the node internal value.
 */
class NegOperation implements Operation {

	/** Singleton instance of this operation */
	public static final NegOperation INSTANCE = new NegOperation();

	/** Hidden constructor */
	private NegOperation() {}

	@Override
	public Shift execute(final Node node) {
		node.negate();
		return Shift.NEXT;
	}
}
