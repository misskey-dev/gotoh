import * as crypto from "node:crypto";
import * as fs from "node:fs/promises";
import * as path from "node:path";
import { describe, expect, it } from "vitest";
import { ZipReader } from "../index.js";

describe(ZipReader.name, () => {
  it("should read zip file", async () => {
    await fs.mkdir(path.resolve(__dirname, "..", "node_modules", ".tmp"), {
      recursive: true,
    });
    const tmp = await fs.mkdtemp(
      path.resolve(__dirname, "..", "node_modules", ".tmp", "zip-")
    );
    try {
      const zip = ZipReader.withDestinationPath(tmp);
      zip.viaBuffer(
        await fs.readFile(
          path.resolve(
            __dirname,
            "resources",
            "zip",
            "sqlite-amalgamation-3410200.zip"
          )
        )
      );
      const files = await fs.readdir(
        path.resolve(tmp, "sqlite-amalgamation-3410200")
      );
      expect(files).toEqual([
        "shell.c",
        "sqlite3.c",
        "sqlite3.h",
        "sqlite3ext.h",
      ]);
      const hashes = await Promise.all(
        files.map(async (file) =>
          crypto
            .createHash("sha256")
            .update(
              await fs.readFile(
                path.resolve(tmp, "sqlite-amalgamation-3410200", file)
              )
            )
            .digest("hex")
        )
      );
      expect(hashes).toEqual([
        "917bb6d7539c2a782bfbfcdec2405a7a27070930ada28ae00607fe1e9b3f3ad1",
        "efe007ec6f1a4084b87cebba18cc887c316e0baaf4ec2675990726b94cb85470",
        "10ff8c8842b5e6bb0555f39ce946b77d0f84acc9e33895d22242444d91900571",
        "cf9785762f8801049b841f873cdeecfa2780b0f6465e2b76f72d2c10384cf7ce",
      ]);
    } finally {
      await fs.rm(tmp, { recursive: true });
    }
  });
});
