package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.ref.InputReference;
import com.kineolyan.tzio.v1.ref.OutputReference;

public class MovOperation implements Operation {

	private final InputReference from;
	private final OutputReference to;

	public MovOperation(final InputReference from, final OutputReference to) {
		this.from = from;
		this.to = to;
	}

	@Override
	public Shift execute(final Node node) {
		if (this.from.canRead(node) && this.to.canWrite(node)) {
			final int value = this.from.readValue(node);
			this.to.writeValue(node, value);
			return Operation.Shift.NEXT;
		} else {
			return Operation.Shift.STAY;
		}
	}
}
