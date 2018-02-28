package com.kineolyan.tzio.ref;

import com.kineolyan.tzio.InputSlot;
import com.kineolyan.tzio.Node;
import com.kineolyan.tzio.OutputSlot;

import java.util.stream.IntStream;

public class SlotReference implements InputReference, OutputReference {

	private static final SlotReference[] CACHE;
	static {
		CACHE = IntStream.range(0, 10)
			.mapToObj(SlotReference::new)
			.toArray(SlotReference[]::new);
	}

	private final int slotIndex;

	public SlotReference(final int slotIndex) {
		this.slotIndex = slotIndex;
	}

	public static SlotReference of(final int id) {
		if (id < CACHE.length) {
			return CACHE[id];
		} else {
			return new SlotReference(id);
		}
	}

	private InputSlot getInput(final Node node) {
		return node.inputSlots[this.slotIndex];
	}

	private OutputSlot getOutput(final Node node) {
		return node.outputSlots[this.slotIndex];
	}

	@Override
	public boolean canRead(final Node node) {
		return getInput(node).canRead();
	}

	@Override
	public int readValue(final Node node) {
		return getInput(node).read();
	}

	@Override
	public boolean canWrite(final Node node) {
		return getOutput(node).canWrite();
	}

	@Override
	public void writeValue(final Node node, final int value) {
		getOutput(node).write(value);
	}
}
