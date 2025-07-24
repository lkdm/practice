namespace RainfallAPI.Models
{
    using System;
    using System.ComponentModel.DataAnnotations;

    public class ModelBase
    {
        public ModelBase()
        {
            this.Id = Guid.NewGuid();
            this.CreatedAt = DateTimeOffset.UtcNow;
            this.ModifiedAt = DateTimeOffset.UtcNow;
        }

        [Key]
        public Guid Id { get; set; }

        public DateTimeOffset CreatedAt { get; set; }

        public DateTimeOffset ModifiedAt { get; set; }
    }
}
