using AutoMapper;
using RainfallAPI.Models;
using RainfallAPI.Dtos;

namespace RainfallAPI.Profiles
{
    public class HabitsProfile : Profile
    {
        public HabitsProfile()
        {
            // Source ->> Target
            CreateMap<Habit, HabitReadDto>();
            CreateMap<HabitCreateDto, Habit>();
            CreateMap<HabitUpdateDto, Habit>();
        }
    }
}