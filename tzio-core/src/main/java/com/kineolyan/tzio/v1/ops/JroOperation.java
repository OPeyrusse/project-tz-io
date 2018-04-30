package com.kineolyan.tzio.v1.ops;

import com.kineolyan.tzio.v1.Node;
import com.kineolyan.tzio.v1.NodeExecution;

import java.util.function.ToIntFunction;

/**
 * Conditional operation offsetting to the "next" operation according to the value of the node.
 */
class JroOperation implements Operation {

	/** Singleton instance of this operation */
	public static JroOperation INSTANCE = new JroOperation();

	/** Hidden constructor */
	private JroOperation() {}

	@Override
	public Shift execute(final Node node) {
		return JroShift.create(node.getAccValue());
	}

	/**
	 * Special increment shifting by a given value.
	 */
	private static class JroShift implements Operation.Shift {

		/** Increment to apply to the operation */
		private final int increment;

		/**
		 * Constructor.
		 * @param increment operation increment
		 */
		private JroShift(final int increment) {
			this.increment = increment;
		}

		/**
		 * Creates the appropriate shift according to the increment.
		 * @param increment increment to apply
		 * @return the increment
		 */
		public static Operation.Shift create(final int increment) {
			if (increment == 0) {
				return Shift.STAY;
			} else if (increment == 1) {
				return Shift.NEXT;
			} else {
				return new JroShift(increment);
			}
		}

		@Override
		public int update(final ToIntFunction<String> labelIndex, final int current, final int max) {
			int target = current + this.increment;
			if (target < 0) {
				while (target < 0) {
					target += max;
				}
			} else {
				target %= max;
			}
			return target;
		}

	}
}
