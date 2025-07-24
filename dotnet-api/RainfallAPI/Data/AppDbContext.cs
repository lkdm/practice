using RainfallAPI.Models;
using Microsoft.EntityFrameworkCore;

namespace RainfallAPI.Data
{
    public class AppDbContext : DbContext
    {
        public AppDbContext(DbContextOptions<AppDbContext> options) : base(options)
        {

        }

        public DbSet<Habit> Habits => Set<Habit>();
    }
}
