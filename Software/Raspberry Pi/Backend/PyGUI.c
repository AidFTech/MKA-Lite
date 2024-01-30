#include "PyGUI.h"

//Initialize the Python interface. Called from main at the start of the program.
wchar_t* pyInit(int argc, char *argv[]) {
	wchar_t* program = Py_DecodeLocale(argv[0], NULL);
	Py_SetProgramName(program);
	Py_Initialize();

	return program;
}

//Finalize the Python interface at the end of the program.
void pyFinalize(wchar_t* program) {
	PyMem_RawFree(program);
	Py_Finalize();
}

//Start the MKA.
PyObject* startMKA(const char* fname) {
	FILE* pyfile = fopen(fname, "r");
	PyRun_SimpleFile(pyfile, fname);

	PyObject* global_dict = PyModule_GetDict(PyImport_AddModule("__main__"));
	PyObject* mka_class = PyDict_GetItemString(global_dict, "MKA");

	PyObject* def_tuple = PyTuple_New(3);
	#ifdef RPI_UART
	PyTuple_SetItem(def_tuple, 0, PyBool_FromLong(1));
	#else
	PyTuple_SetItem(def_tuple, 0, PyBool_FromLong(0));
	#endif
	PyTuple_SetItem(def_tuple, 1, PyBool_FromLong(0));
	PyTuple_SetItem(def_tuple, 2, Py_BuildValue("s", fname));

	PyObject* mka = PyObject_CallObject(mka_class, def_tuple);
	fclose(pyfile);
		
	return mka;
}