package com.kineolyan.tzio.v1;

public interface OutputSlot {

	boolean canWrite();

	void write(int value);

}
