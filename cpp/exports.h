//Credits to libTesseract: https://github.com/ollydev/libTesseract/blob/master/exports.h
#ifndef EXPORTS_INCLUDED
#define EXPORTS_INCLUDED

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#endif

#if defined(_WIN32) || defined(_WIN64)
#define EXPORT __declspec(dllexport)
extern HMODULE module;
#else
#include <dlfcn.h>
#define EXPORT [[gnu::visibility("default")]]
#endif

static const char* PascalExports[] =
{
	(char*)"HelloWorld", (char*)"procedure HelloWorld();",
};

static const char* PascalTypes[] =
{
	(char*)"PHelloChar", (char*)"^Char;",
};

static const long int PascalExportCount = sizeof(PascalExports) / (sizeof(PascalExports[0]) * 2);
static const long int PascalTypeCount = sizeof(PascalTypes) / (sizeof(PascalTypes[0]) * 2);

extern "C"
{
	EXPORT void HelloWorld();

	EXPORT int GetPluginABIVersion();
	EXPORT int GetFunctionCount();
	EXPORT int GetTypeCount();
	EXPORT int GetFunctionInfo(int Index, void** Address, char** Definition);
	EXPORT int GetTypeInfo(int Index, char** Type, char** Definition);
}

#endif
