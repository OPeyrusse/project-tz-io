package com.kineolyan.tzio.ops;

import com.kineolyan.tzio.Node;

public class LabelOperation implements Operation {

	private final String label;

	public LabelOperation(final String label) {
		this.label = label;
	}

	@Override
	public String label() {
		return this.label;
	}

	@Override
	public Shift execute(final Node node) {
		throw new UnsupportedOperationException("Cannot execute a label operation");
	}
}
