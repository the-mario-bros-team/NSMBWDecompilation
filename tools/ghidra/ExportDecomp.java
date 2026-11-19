// Export every function's decompiled C plus an index.
// Args: <outputDir>
// @category REA
import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;

import java.io.File;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class ExportDecomp extends GhidraScript {
    @Override
    public void run() throws Exception {
        String[] args = getScriptArgs();
        File outDir = new File(args.length > 0 ? args[0] : "decompiled");
        if (!outDir.exists() && !outDir.mkdirs()) {
            throw new RuntimeException("cannot create " + outDir);
        }

        DecompInterface ifc = new DecompInterface();
        if (!ifc.openProgram(currentProgram)) {
            throw new RuntimeException("decompiler could not open program");
        }

        FunctionManager fm = currentProgram.getFunctionManager();
        PrintWriter index = new PrintWriter(new File(outDir, "functions.tsv"), "UTF-8");
        index.println("address\tname\tsize\tdecompiled\tfile");

        int count = 0;
        int ok = 0;
        for (Function f : fm.getFunctions(true)) {
            count++;
            String addr = f.getEntryPoint().toString();
            String name = f.getName();
            long size = f.getBody().getNumAddresses();

            DecompileResults res = ifc.decompileFunction(f, 120, monitor);
            boolean good = res != null && res.decompileCompleted() && res.getDecompiledFunction() != null;
            String file = "";
            if (good) {
                String c = res.getDecompiledFunction().getC();
                String base = (addr + "_" + name).replaceAll("[^A-Za-z0-9._-]", "_");
                if (base.length() > 120) base = base.substring(0, 120);
                file = base + ".c";
                Path p = Paths.get(outDir.getAbsolutePath(), file);
                Files.write(p, c.getBytes(StandardCharsets.UTF_8));
                ok++;
            }
            index.println(addr + "\t" + name + "\t" + size + "\t" + good + "\t" + file);
        }
        index.close();
        ifc.dispose();
        println("EXPORTED " + ok + "/" + count + " functions to " + outDir.getAbsolutePath());
    }
}
