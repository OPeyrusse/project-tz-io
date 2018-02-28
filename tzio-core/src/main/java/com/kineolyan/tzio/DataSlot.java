package com.kineolyan.tzio;

public class DataSlot implements InputSlot, OutputSlot, TransactionalElement {

	private int value = 0;
	private boolean hasValue = false;
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
