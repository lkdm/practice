using System.ComponentModel.DataAnnotations;

namespace RainfallAPI.Models
{
    public class Person: ModelBase
    {
        public Person()
        {
            this.Stations = new List<Station>();
        }

        public Person(string givenName, string familyName)
            : this()
        {
            this.GivenName = givenName;
            this.FamilyName = familyName;
        }

        [Required]
        public string GivenName { get; set; }

        public string FamilyName { get; set; } = null!;

        public string AdditionalNames { get; set; } = null!;

        [Required]
        [EmailAddress]
        public string EmailAddress { get; set; }

        /// <summary>
        /// A list of Stations the person maintains.
        /// </summary>
        public virtual List<Station> Stations { get; set; }
    }
}