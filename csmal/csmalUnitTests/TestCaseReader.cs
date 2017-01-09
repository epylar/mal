using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace csmalUnitTests
{
    static class TestCaseReader
    {
        public static Case[] ReadCases(string tests)
        {
            Case[] cases = ReadCasesList(tests).ToArray();
            return cases;
        }

        public static List<Case> ReadCasesList(string tests)
        {
            var cases = new List<Case>();
            var lines = Regex.Split(tests, "\r\n|\r|\n");
            List<String> inputs = new List<string>();
            bool nextInputResets = false;

            var theCase = new Case();

            foreach (string line in lines)
            {
                if (line.StartsWith(";;") || line.StartsWith(" ;;"))
                {
                    continue; // comment
                }

                if (line.StartsWith("; "))
                {
                    // could be an expected output line, but TODO we don't handle this now
                    nextInputResets = true;
                    continue;
                }

                if (String.IsNullOrWhiteSpace(line))
                {
                    continue; // no content
                }

                if (line.StartsWith(";=>"))
                {
                    if (!inputs.Any())
                    {
                        throw new Exception("bad format: output " + line + " before specifying input");
                    }
                    var output = line.Substring(3);
                    theCase.Add(new Step {Input = inputs, Output = output});
                    inputs = new List<string>();
                }
                else
                {
                    if (nextInputResets)
                    {
                        nextInputResets = false;
                        inputs = new List<string>();
                    }
                    inputs.Add(line);
                }
            }
            cases.Add(theCase); // FIXME need to split into actual sets of steps
            return cases;
        }
    }
}
