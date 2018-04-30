package com.kineolyan.tzio.v1.execs;

import com.kineolyan.tzio.v1.TzEnv;

/**
 * Interface for classes making a {@link TzEnv} run.
 * <p>
 *   This feeds it with inputs and handles the produced outputs.
 * </p>
 */
public interface TzExecutor {

	/**
	 * Runs a given environment.
	 * @param env environment to run.
	 */
	void run(TzEnv env);

}
