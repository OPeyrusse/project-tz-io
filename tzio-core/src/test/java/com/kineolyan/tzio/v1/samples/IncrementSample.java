package com.kineolyan.tzio.v1.samples;

import com.kineolyan.tzio.v1.TzEnv;
import com.kineolyan.tzio.v1.ops.Operations;
import com.kineolyan.tzio.v1.ref.References;

import java.util.List;

/**
 * Tests class implementing an node increment the input by one.
 */
public class IncrementSample extends TzEnv {

	private IncrementSample() {
		super();
		withSlots(2, new int[]{0}, new int[]{1});
		addNode(
			"1",
			1,
			new int[]{0},
			new int[]{1},
			List.of(
				Operations.MOV(References.inSlot(1), References.acc()),
				Operations.ADD(References.value(1)),
				Operations.MOV(References.acc(), References.outSlot(1))));
	}

	public static void main(final String[] args) {
		new IncrementSample().runFromSystem(args);
	}

}
