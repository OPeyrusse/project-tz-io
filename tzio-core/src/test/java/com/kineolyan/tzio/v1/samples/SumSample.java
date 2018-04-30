package com.kineolyan.tzio.v1.samples;

import com.kineolyan.tzio.v1.TzEnv;
import com.kineolyan.tzio.v1.ops.Operations;
import com.kineolyan.tzio.v1.ref.References;

import java.util.List;

/**
 * Tests class implementing an node increment the input by one.
 */
public class SumSample extends TzEnv {

	private SumSample() {
		super();
		withSlots(5, new int[]{0, 1}, new int[]{4});
		addNode(
			"1",
			1,
			new int[]{0},
			new int[]{2},
			List.of(
				Operations.MOV(References.inSlot(1), References.acc()),
				Operations.ADD(References.acc()),
				Operations.MOV(References.acc(), References.outSlot(1))));
		addNode(
			"2",
			1,
			new int[]{1},
			new int[]{3},
			List.of(
				Operations.MOV(References.inSlot(1), References.acc()),
				Operations.ADD(References.value(1)),
				Operations.MOV(References.acc(), References.outSlot(1))));
		addNode(
			"3",
			1,
			new int[]{2, 3},
			new int[]{4},
			List.of(
				Operations.MOV(References.inSlot(1), References.acc()),
				Operations.ADD(References.inSlot(2)),
				Operations.MOV(References.acc(), References.outSlot(1))));
	}

	public static void main(final String[] args) {
		new SumSample().runFromSystem(args);
	}

}
