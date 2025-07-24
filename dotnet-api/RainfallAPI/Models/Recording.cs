using System.ComponentModel.DataAnnotations;

namespace RainfallAPI.Models
{
    public class Recording : ModelBase
    {
        public double Amount { get; set; }

        public Unit Unit { get; set; }

        /// <summary>
        /// The date and time in which the rainfall was recorded.
        /// Used because meteorologists may add recordings in batches.
        /// </summary>
        public DateTimeOffset RecordedAt { get; set; }

        public Guid StationId { get; set; }
        public virtual Station Station { get; set; }
    }
}