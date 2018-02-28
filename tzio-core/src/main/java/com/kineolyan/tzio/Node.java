package com.kineolyan.tzio;

import com.kineolyan.tzio.ref.InputReference;
import com.kineolyan.tzio.ref.OutputReference;

public class Node {

	public final InputSlot[] inputSlots;
	public final OutputSlot[] outputSlots;
	private int accValue;
	private final int[] memorySlots;

	public Node(
		final int memorySize,
		final InputSlot[] inputSlots,
		final OutputSlot[] outputSlots) {
		this.memorySlots = new int[memorySize];
		this.accValue = 0;
		this.inputSlots = inputSlots;
		this.outputSlots = outputSlots;
	}

	public int getAccValue() {
		return this.accValue;
	}

	public void setAccValue(final int value) {
		this.accValue = value;
	}

	// Operations

	public final void moveValue(final InputReference from, final OutputReference to) {
		final int value = from.readValue(this);
		to.writeValue(this, value);
	}

	public final void addValue(final InputReference source) {
		changeValue(source.readValue(this));
	}

	public final void subValue(final InputReference source) {
		changeValue(-source.readValue(this));
	}

	private final void changeValue(int value) {
		this.accValue += value;
	}

	public final void bakValue(final int memorySlot) {
		this.memorySlots[memorySlot] = this.accValue;
	}

	public final void swapValue(final int memorySlot) {
		final int swp = this.accValue;
		this.accValue = this.memorySlots[memorySlot];
		this.memorySlots[memorySlot] = swp;
	}

}
