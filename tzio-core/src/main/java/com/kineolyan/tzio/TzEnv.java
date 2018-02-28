package com.kineolyan.tzio;

import java.util.Map;

public class TzEnv {

	public final Map<String, Node> nodes;
	public final DataSlot[] slots;

	public TzEnv(
		final Map<String, Node> nodes,
		final int slotCount,
		final Map<Integer, DataSlot> externalSlots) {
		this.nodes = nodes;
		this.slots = new DataSlot[slotCount];
		for (int i = 0; i < slotCount; i += 1) {
			final DataSlot slot = externalSlots.get(i);
			this.slots[i] = slot != null ? slot : new DataSlot();
		}
	}

}
