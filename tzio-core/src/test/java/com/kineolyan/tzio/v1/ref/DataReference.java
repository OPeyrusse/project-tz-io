package com.kineolyan.tzio.v1.ref;

import com.kineolyan.tzio.v1.Node;

import java.util.OptionalInt;

/**
 * Output reference for tests storing the value.
 */
public class DataReference implements OutputReference {

	/** Internal value */
	private OptionalInt value;

	public DataReference() {
		this.value = OptionalInt.empty();
	}

	public OptionalInt getValue() {
		return this.value;
	}

	@Override
	public boolean canWrite(Node node) {
		return true;
	}

	@Override
	public void writeValue(Node node, int value) {
		this.value = OptionalInt.of(value);
	}

	public void reset() {
		this.value = OptionalInt.empty();
	}
}

