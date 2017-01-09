using System;

namespace csmal
{
    internal class SymbolNotFoundException : Exception
    {
        public SymbolNotFoundException(string message, Exception innerException) : base(message, innerException)
        {
        }
    }
}