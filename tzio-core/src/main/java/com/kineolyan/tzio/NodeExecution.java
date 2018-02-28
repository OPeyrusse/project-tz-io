package com.kineolyan.tzio;

import com.kineolyan.tzio.ops.Operation;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class NodeExecution {

	private final Node node;
	private final List<Operation> operations;
	private final Map<String, Integer> labels;
	private int stepIdx;

	public NodeExecution(
		final Node node,
		final List<Operation> operations) {
		this.node = node;
		this.operations = new ArrayList<>();
		this.labels = new HashMap<>();
		this.stepIdx = 0;

		for (final Operation operation : operations) {
			if (operation.label() != null) {
				this.labels.put(operation.label(), this.operations.size());
			} else {
				this.operations.add(operation);
			}
		}
	}

	public void runStep() {
		final Operation operation = this.operations.get(this.stepIdx);
		final Operation.Shift nextOperation = operation.execute(this.node);
		this.stepIdx = nextOperation.update(this, this.stepIdx, this.operations.size());
	}

	public int getLabelOperationIdx(final String targetLabel) {
		return this.labels.computeIfAbsent(
			targetLabel,
			key -> { throw new IllegalArgumentException("No label named " + targetLabel); });
	}
}
