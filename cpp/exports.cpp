//Credits to libTesseract: https://github.com/ollydev/libTesseract/blob/master/exports.cpp

#include "exports.h"
#include <iostream>
#include <cstring>

void HelloWorld()
{
    std::cout << "Hello World!" << std::endl;
}

int GetPluginABIVersion()
{
	return 2;
}

int GetFunctionCount()
{
	return PascalExportCount;
}

int GetTypeCount()
{
	return PascalTypeCount;
}

int GetFunctionInfo(int Index, void** Address, char** Definition)
{
	if (Index < PascalExportCount)
	{
		#if defined(_WIN32) || defined(_WIN64)
		*Address = (void*)GetProcAddress(module, PascalExports[Index * 2]);
		#else
		*Address = (void*)dlsym(RTLD_DEFAULT, PascalExports[Index * 2]);
		#endif
		strcpy(*Definition, PascalExports[Index * 2 + 1]);

		return Index;
	}

	return -1;
}

int GetTypeInfo(int Index, char** Type, char** Definition)
{
	if (Index < PascalTypeCount)
	{
		strcpy(*Type, PascalTypes[Index * 2 + 0]);
		strcpy(*Definition, PascalTypes[Index * 2 + 1]);

		return Index;
	}

	return -1;
}
