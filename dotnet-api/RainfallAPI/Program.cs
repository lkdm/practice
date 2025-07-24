using Microsoft.EntityFrameworkCore;
using RainfallAPI.Models;
using RainfallAPI.Data;
using AutoMapper;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();


// await using var conn = new NpgsqlConnection(builder.Configuration.GetConnectionString("SQLDbConnection"));
// await conn.OpenAsync();

// Connect to PostgreSQL Database
var connectionString = builder.Configuration.GetConnectionString("DefaultConnection");
builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseNpgsql(connectionString));

// Dependency injection– if interface is requested, give them a concrete implementation.
builder.Services.AddScoped<IHabitRepo, HabitRepo>();

builder.Services.AddAutoMapper(AppDomain.CurrentDomain.GetAssemblies());

var app = builder.Build();
//... rest of the code omitted for brevity

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

// Endpoints
app.MapGet("api/v2022q4/status", () => Results.Ok("Okay"));

app.MapGet("api/v2022q4/habits", async (IHabitRepo repo, IMapper mapper) =>
{
    var habits = await repo.GetAllHabits();
    return Results.Ok(habits);
});


app.Logger.LogInformation("👉 The application started");
app.Run();