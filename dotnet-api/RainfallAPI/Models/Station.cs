using System.ComponentModel.DataAnnotations;

namespace RainfallAPI.Models
{
    /// <summary>
    /// Station is the instrument used to collect data for the recording.
    /// Most often, this is a rain gauge.
    /// </summary>
    public class Station: ModelBase
    {
        public Station()
        {
            this.Recordings = new List<Recording>();
        }

        [MaxLength(255)]
        public string? Description { get; set; }

        [Required]
        public double Lat { get; set; }

        [Required]
        public double Lon { get; set; }

        public virtual List<Recording> Recordings { get; set; }

        public Guid PersonId { get; set; }
        public virtual Person Person { get; set; }
    }
}