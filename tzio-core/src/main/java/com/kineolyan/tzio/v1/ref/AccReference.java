package com.kineolyan.tzio.v1.ref;

import com.kineolyan.tzio.v1.Node;

public class AccReference implements InputReference, OutputReference {

	@Override
	public boolean canRead(final Node node) {
		return true;
	}

	@Override
	public int readValue(final Node node) {
		return node.getAccValue();
	}

	@Override
	public boolean canWrite(final Node node) {
		return true;
	}

	@Override
	public void writeValue(final Node node, final int value) {
		node.setAccValue(value);
	}
}
