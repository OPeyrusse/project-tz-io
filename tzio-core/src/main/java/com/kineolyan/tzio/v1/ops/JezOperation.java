package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.NodeExecution;

/**
 * Conditional operation jumping to a given label when the consider node value is 0.
 */
public class JezOperation implements Operation, Operation.Shift {

	/** Target label operation */
	private String targetLabel;

	/**
	 * Constructor
	 * @param label label to go to when the value is 0
	 */
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
