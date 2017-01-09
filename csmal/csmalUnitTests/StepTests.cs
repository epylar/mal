using System.Collections.Generic;
using System.IO;
using csmal;
using NUnit.Framework;

namespace csmalUnitTests
{
    [TestFixture]
    public class StepTests
    {
        private readonly List<string> _rawTests = new List<string>()
        {
            "step0_repl.mal",
            "step1_read_print.mal",
            "step2_eval.mal",
            "step3_env.mal"
        };

        private const string DirPrefix = @"..\..\..\..\tests\";

        public string GetRawTextFromTestName(string testName)
        {
            return File.ReadAllText(DirPrefix + testName);
        }

        public Case[] Step0Cases()
        {
            return TestCaseReader.ReadCases(GetRawTextFromTestName(_rawTests[0]));
        }

        public Case[] Step1Cases()
        {
            return TestCaseReader.ReadCases(GetRawTextFromTestName(_rawTests[1]));
        }

        public Case[] Step2Cases()
        {
            return TestCaseReader.ReadCases(GetRawTextFromTestName(_rawTests[2]));
        }

        public Case[] Step3Cases()
        {
            return TestCaseReader.ReadCases(GetRawTextFromTestName(_rawTests[3]));
        }

        [Test, TestCaseSource("Step0Cases")]
        public void Check_Step0Cases(Case theSteps)
        {
            CaseChecker.CheckCase(theSteps, Step0Repl.Rep);
        }

        [Test, TestCaseSource("Step1Cases")]
        public void Check_Step1Cases(Case theSteps)
        {
            CaseChecker.CheckCase(theSteps, Step1ReadPrint.Rep);
        }

        [Test, TestCaseSource("Step2Cases")]
        public void Check_Step2Cases(Case theSteps)
        {
            CaseChecker.CheckCase(theSteps, Step2Eval.Rep);
        }

        [Test, TestCaseSource("Step3Cases")]
        public void Check_Step3Cases(Case theSteps)
        {
            CaseChecker.CheckCase(theSteps, (Step3Env.Make()).Rep);
        }
    }
}
