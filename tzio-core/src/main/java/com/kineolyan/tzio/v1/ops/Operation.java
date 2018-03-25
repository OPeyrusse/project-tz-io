package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.NodeExecution;

public interface Operation {

	default String label() {
		return null;
	}

	Shift execute(final Node node);

	interface Shift {

		Operation.Shift NEXT = (Operation.Shift) (exec, current, max) -> (current + 1) % max;
		Operation.Shift STAY = (Operation.Shift) (exec, current, max) -> current;

		int update(NodeExecution execution, int current, int max);

	}

}
