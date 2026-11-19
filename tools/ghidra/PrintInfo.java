// Ghidra script: print program identity after import.
// @category REA
import ghidra.app.script.GhidraScript;

public class PrintInfo extends GhidraScript {
    @Override
    public void run() throws Exception {
        println("PROGRAM: " + currentProgram.getName());
        println("LANGUAGE: " + currentProgram.getLanguageID().getIdAsString());
        println("PROCESSOR: " + currentProgram.getLanguage().getProcessor().toString());
        println("IMAGEBASE: " + currentProgram.getImageBase().toString());
        println("FUNCTIONS: " + currentProgram.getFunctionManager().getFunctionCount());
        println("MEMORY_BLOCKS: " + currentProgram.getMemory().getBlocks().length);
    }
}
