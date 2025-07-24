using Microsoft.EntityFrameworkCore;
using RainfallAPI.Models;


namespace RainfallAPI.Data;

public class HabitRepo : IHabitRepo
{
    private readonly AppDbContext _context;
    public HabitRepo(AppDbContext context)
    {
        _context = context;
    }
    public async Task CreateHabit(Habit habit)
    {
        if (habit == null)
        {
            throw new ArgumentNullException(nameof(habit));
        }
        await _context.AddAsync(habit);
    }

    public void DeleteHabit(Habit habit)
    {
        if (habit == null)
        {
            throw new ArgumentNullException(nameof(habit));
        }
        _context.Habits.Remove(habit);
    }

    public async Task<IEnumerable<Habit>> GetAllHabits()
    {

        return await _context.Habits.ToListAsync();
    }

    public async Task<Habit?> GetHabitById(Guid id)
    {
        return await _context.Habits.FirstOrDefaultAsync(c => c.Id == id);
    }

    public async Task SaveChanges()
    {
        await _context.SaveChangesAsync();
    }
}