package com.kineolyan.tzio.v1;

import com.kineolyan.tzio.v1.ref.InputReference;
import com.kineolyan.tzio.v1.ref.OutputReference;

import java.util.function.IntPredicate;
import java.util.function.Predicate;

/**
 * Implementation of a node in the TZ-IO environment.
 * <p>
 *   A node is composed of a series of inputs it can read, outputs it can write into,
 *   a variable length internal memory and a given current value.
 * </p>
 */
public class Node {

	/** Node inputs */
	private final InputSlot[] inputSlots;
	/** Node outputs */
	private final OutputSlot[] outputSlots;
	/** Node internal value */
	private int accValue;
	/** Node memory slots, usable for value keeping */
	private final int[] memorySlots;

	/**
	 * Constructor.
	 * @param memorySize size of the internal memory
	 * @param inputSlots node inputs
	 * @param outputSlots node outputs
	 */
	public Node(
		final int memorySize,
		final InputSlot[] inputSlots,
		final OutputSlot[] outputSlots) {
		this.memorySlots = new int[memorySize];
		this.accValue = 0;
		this.inputSlots = inputSlots;
		this.outputSlots = outputSlots;
	}

	/**
	 * Reads the node value.
	 * @return node value
	 */
	public int getAccValue() {
		return this.accValue;
	}

	/**
	 * Sets the node value to a given input
	 * @param value new value for the node
	 */
	public void setAccValue(final int value) {
		this.accValue = value;
	}

	/**
	 * Access a given input of the node.
	 * @param idx 0-based index of the input in the node.
	 * @return the slot
	 */
	public InputSlot getInput(final int idx) {
		return this.inputSlots[idx];
	}

	/**
	 * Access a given output of the node.
	 * @param idx 0-based index of the output in the node.
	 * @return the slot
	 */
	public OutputSlot getOutput(final int idx) {
		return this.outputSlots[idx];
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

	public final void negate() {
		this.accValue = -this.accValue;
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

	public final boolean testValue(final IntPredicate predicate) {
		return predicate.test(this.accValue);
	}

}
