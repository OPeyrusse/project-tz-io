package com.kineolyan.tzio;

public interface OutputSlot {

	boolean canWrite();

	void write(int value);

}
