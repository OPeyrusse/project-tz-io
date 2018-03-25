package com.kineolyan.tzio.v1;

/**
 * Input/Output slot storing a single value.
 */
public class DataSlot implements InputSlot, OutputSlot, TransactionalElement {

	/** Value in the slot */
	private int value = 0;
	/** Flag marking that a value is currently stored */
	private boolean hasValue = false;
	/** Flag marking that the value has been consumed */
	private boolean hasValueAfterStep = false;

	@Override
	public boolean canRead() {
		return this.hasValue;
	}

	@Override
	public int read() {
		this.hasValueAfterStep = false;
		return this.value;
	}

	@Override
	public boolean canWrite() {
		return !this.hasValue;
	}

	@Override
	public void write(final int value) {
		this.value = value;
		this.hasValueAfterStep = true;
	}

	@Override
	public void onStepEnd() {
		this.hasValue = this.hasValueAfterStep;
	}
}
