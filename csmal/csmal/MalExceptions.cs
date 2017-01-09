using System;

namespace csmal
{
    internal class WrongNumberOfArgumentsException : Exception
    {
        public WrongNumberOfArgumentsException(string message, Exception innerException)
            : base(message, innerException)
        {
        }
    }

    internal class NoTokensException : Exception
    {
        public NoTokensException(string message, Exception innerException) : base(message, innerException)
        {
        }
    }
}