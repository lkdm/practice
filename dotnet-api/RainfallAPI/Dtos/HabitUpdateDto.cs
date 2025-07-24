using System.ComponentModel.DataAnnotations;

namespace RainfallAPI.Dtos
{
    public class HabitUpdateDto
    {
        [MaxLength(1)]
        public string? Icon { get; set; }

        [Required]
        public string? Name { get; set; }
    }
}