namespace RainfallAPI.Models
{
    using System;

    /// <summary>
    /// Rain gauges can be measured in millimetres or inches, depending on the locale.
    /// </summary>

    public enum Unit : int
    {
        Millimetres = 0,
        Inches = 1
    }
}
