package com.kineolyan.tzio.v1.ref;

/**
 * Public facade to create references to inputs and outputs.
 */
public class References {

	private References() {}

	public static InputReference inSlot(final int idx) {
		return SlotReference.of(idx);
	}

	public static OutputReference outSlot(final int idx) {
		return SlotReference.of(idx);
	}

	public static AccReference acc() {
		return AccReference.INSTANCE;
	}

	public static InputReference value(final int value) {
		return ValueReference.of(value);
	}

}
