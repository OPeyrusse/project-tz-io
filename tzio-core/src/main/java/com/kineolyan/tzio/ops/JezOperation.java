package com.kineolyan.tzio.ops;

import com.kineolyan.tzio.Node;
import com.kineolyan.tzio.NodeExecution;

public class JezOperation implements Operation, Operation.Shift {

	private String targetLabel;

	public JezOperation(final String label) {
		this.targetLabel = label;
	}

	@Override
	public Shift execute(final Node node) {
		if (node.getAccValue() == 0) {
			return this;
		} else {
			return Shift.NEXT;
		}
	}

	@Override
	public int update(final NodeExecution execution, final int current, final int max) {
		return execution.getLabelOperationIdx(this.targetLabel);
	}
}
