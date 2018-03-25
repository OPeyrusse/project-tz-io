package com.kineolyan.tzio.v1;

import java.util.Queue;
import java.util.concurrent.LinkedBlockingDeque;

public class InputQueueSlot implements InputSlot, TransactionalElement {

	private Queue<Integer> values = new LinkedBlockingDeque<>();

	/**
	 * Adds a new value to the input queue.
	 * @param value value to add
	 */
	public void enqueue(final int value) {
		this.values.offer(value);
	}

	@Override
	public boolean canRead() {
		return !this.values.isEmpty();
	}

	@Override
	public int read() {
		return this.values.poll();
	}

	@Override
	public void onStepEnd() {
		// Nothing to do
	}
}
