using RainfallAPI.Models;

namespace RainfallAPI.Data
{
    public interface IHabitRepo
    {
        Task SaveChanges();
        Task<Habit?> GetHabitById(Guid Id);
        Task<IEnumerable<Habit>> GetAllHabits();
        Task CreateHabit(Habit habit);

        void DeleteHabit(Habit habit);

    }
}

