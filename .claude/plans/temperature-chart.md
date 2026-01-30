# Implementation Plan: Temperature Components Chart

## Summary

Add a temperature monitoring chart to the web dashboard displaying CPU, NVMe, GPU, and other component temperatures over time. **No database changes required** - temperature data is already being collected and stored.

## Phase 1: Investigation Results ✓

### Temperature Data Storage (CONFIRMED)
- **Already saved**: Temperature data is stored in the `metrics` table as JSON array
- **Structure**: `temperatures: Option<String>` containing serialized `Vec<Temperature>`
- **Temperature type**: `{ sensor: String, value: f32, label: String }`
- **Collection**: Every 2 seconds from `/sys/class/hwmon/` (k10temp, nvme, GPU sensors)
- **Availability**: Already part of `SystemMetrics` returned by `/api/v1/metrics/history` and WebSocket

### Existing Chart Pattern (IDENTIFIED)
- **Library**: Recharts (LineChart, AreaChart components)
- **Data flow**:
  - `useMetrics` hook fetches initial 60 records via REST
  - WebSocket provides real-time updates
  - Components receive `history: SystemMetrics[]` prop
- **Pattern examples**:
  - `CpuChart.tsx` - transforms history to extract CPU usage over time
  - `MemoryChart.tsx` - similar pattern for memory data
- **Chart features**: Responsive container, tooltips, legends, grid, time-based X-axis

## Phase 2: Implementation Design

### 2.1 Component Structure

**File**: `web-frontend/src/components/TemperatureChart.tsx`

```typescript
interface TemperatureChartProps {
  history: SystemMetrics[];  // Same pattern as existing charts
  isDarkMode: boolean;       // For theme-aware styling
}
```

**Chart visualization approach**:
- Line chart showing temperature trends over time
- Multiple lines for different sensors (CPU, NVMe, GPU, etc.)
- Color-coded by sensor type
- Y-axis: Temperature (°C)
- X-axis: Time (last 2 minutes based on 60 records at 2s intervals)

### 2.2 Data Transformation Logic

**Challenge**: Temperature data is an array per metric snapshot
```typescript
// Single snapshot has:
temperatures: [
  { sensor: "k10temp", value: 45.5, label: "CPU" },
  { sensor: "nvme", value: 38.0, label: "NVMe" },
  { sensor: "amdgpu", value: 42.0, label: "GPU" }
]
```

**Solution**: Transform to time-series format
```typescript
// Transform from:
history: SystemMetrics[] (60 snapshots)

// To:
chartData: Array<{
  timestamp: string,
  cpu: number,
  nvme: number,
  gpu: number,
  // ... dynamic sensor entries
}>
```

**Implementation strategy**:
1. Extract unique sensor types from all history records
2. For each timestamp, map sensor values by type
3. Handle missing sensors gracefully (some may not exist on all systems)
4. Calculate max/avg if multiple sensors of same type

### 2.3 Sensor Detection & Mapping

**Dynamic sensor detection**:
- Parse all temperature entries to find available sensors
- Group by label (e.g., all "CPU" sensors together)
- Assign colors from theme-aware palette
- Show only sensors that exist on current system

**Sensor priority** (based on collector implementation):
1. CPU (k10temp, coretemp)
2. NVMe drives
3. GPU (amdgpu, nvidia)
4. Other hwmon sensors

### 2.4 Chart Configuration

**Recharts setup**:
- ResponsiveContainer (width: 100%, height: 300px)
- LineChart with time-based data
- XAxis: Format timestamps as "HH:mm:ss"
- YAxis: Temperature range (auto-scale or 0-100°C)
- Tooltip: Show all sensor temps at point
- Legend: Sensor names with color indicators
- Grid: Subtle background grid
- Lines: Smooth curves with strokeWidth: 2

**Color scheme** (theme-aware):
- CPU: Red/Orange (hot component)
- NVMe: Blue (cooler)
- GPU: Purple
- Others: Green, Yellow, Cyan

### 2.5 Integration Points

**Dashboard.tsx changes**:
```typescript
// Add after MemoryChart section
<div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
  <h2 className="text-xl font-semibold mb-4 text-gray-900 dark:text-white">
    Temperature
  </h2>
  <TemperatureChart history={metricsHistory} isDarkMode={isDarkMode} />
</div>
```

**No TypeScript type changes needed**:
- `SystemMetrics` already includes `temperatures?: Temperature[]`
- Type definition already exists in `web-frontend/src/types/metrics.ts`

## Phase 3: Implementation Steps

### Step 1: Create TemperatureChart Component
**File**: `web-frontend/src/components/TemperatureChart.tsx`

**Tasks**:
1. Create component boilerplate following CpuChart pattern
2. Implement data transformation logic (history → chart data)
3. Detect unique sensors dynamically
4. Configure Recharts LineChart
5. Apply theme-aware colors
6. Add responsive container

**Estimated complexity**: Medium (similar to existing charts but needs dynamic sensor handling)

### Step 2: Integrate into Dashboard
**File**: `web-frontend/src/components/Dashboard.tsx`

**Tasks**:
1. Import TemperatureChart component
2. Add new section in grid layout (after Memory chart)
3. Pass `metricsHistory` and `isDarkMode` props
4. Verify grid layout maintains 2-column responsive design

**Estimated complexity**: Low (simple addition)

### Step 3: Test Edge Cases
**Testing scenarios**:
1. System with no temperature sensors (handle empty array)
2. System with only CPU temp (show single line)
3. System with multiple sensors (show all lines)
4. WebSocket updates (verify real-time chart update)
5. Dark mode toggle (verify colors switch correctly)
6. Empty history on initial load (show empty state or "loading")

**Estimated complexity**: Low (follow existing patterns)

### Step 4: Visual Polish
**Enhancements**:
1. Add critical temperature threshold line (e.g., 85°C in red)
2. Highlight abnormal temperature spikes
3. Show current temperature values in chart title
4. Add tooltip formatting (e.g., "45.5°C" instead of "45.5")
5. Ensure consistent spacing with other charts

**Estimated complexity**: Low (CSS and formatting)

## Phase 4: Technical Considerations

### 4.1 Performance
- **Memoization**: Use `useMemo` for data transformation (only recalculate when history changes)
- **Sensor limit**: If >10 sensors, consider showing only top 5 by temperature
- **Chart re-renders**: Recharts handles efficiently with proper key props

### 4.2 Error Handling
- Handle `temperatures: undefined` or `temperatures: []`
- Show friendly message if no sensors available: "Temperature monitoring not available on this system"
- Log missing sensor data to console (development only)

### 4.3 Accessibility
- Add aria-labels to chart
- Ensure color contrast meets WCAG standards
- Provide text alternative for screen readers

### 4.4 Responsive Design
- Maintain 2-column grid on desktop (lg:grid-cols-2)
- Stack vertically on mobile
- Ensure chart is readable on small screens (min-height: 250px)

## Phase 5: Testing Strategy

### Unit Testing (Optional)
- Test data transformation function with various sensor configurations
- Verify sensor detection logic
- Test theme color mapping

### Integration Testing
1. Start collector service
2. Verify temperature data in API response: `curl http://localhost:5253/api/v1/metrics/current | jq .data.temperature`
3. Load dashboard and confirm chart appears
4. Wait 2 minutes, verify historical data shows
5. Toggle dark mode, verify colors update
6. Open WebSocket inspector, confirm real-time updates

### Browser Testing
- Chrome (confirmed working via previous tests)
- Firefox
- Safari (if available)

## Phase 6: Files to Modify

### New Files
1. **`web-frontend/src/components/TemperatureChart.tsx`** - Main chart component (~150 lines)

### Modified Files
1. **`web-frontend/src/components/Dashboard.tsx`**
   - Add import: `import TemperatureChart from './TemperatureChart';`
   - Add chart section (lines ~120-127, after MemoryChart)

### No Changes Needed
- ❌ Backend collector (already collecting temperature)
- ❌ Database schema (already storing as JSON)
- ❌ API endpoints (already returning temperature data)
- ❌ TypeScript types (Temperature type already defined)
- ❌ useMetrics hook (already fetching temperature data)

## Phase 7: Implementation Order

1. ✅ **Investigation** (COMPLETED)
   - Confirmed temperature data is already available
   - Identified existing chart patterns

2. **Create Component** (~30 minutes)
   - Write TemperatureChart.tsx
   - Implement data transformation
   - Configure Recharts

3. **Integrate** (~5 minutes)
   - Add to Dashboard.tsx
   - Test rendering

4. **Test & Polish** (~15 minutes)
   - Verify WebSocket updates
   - Test dark mode
   - Adjust colors/spacing

5. **Documentation** (~5 minutes)
   - Update USER_GUIDE.md to mention temperature monitoring
   - Add screenshot comment in code

**Total estimated time**: ~1 hour

## Success Criteria

✅ Temperature chart displays on dashboard
✅ Shows all available system sensors (CPU, NVMe, GPU, etc.)
✅ Updates in real-time via WebSocket (every 2 seconds)
✅ Displays 2-minute historical trend (60 data points)
✅ Works correctly in both light and dark modes
✅ Responsive on mobile and desktop
✅ Handles systems with no temperature sensors gracefully
✅ Follows same visual style as existing charts

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| System has no sensors | Medium | Low | Show "Not available" message |
| Too many sensors clutter chart | Low | Medium | Show max 5 sensors, prioritize by importance |
| Chart performance with 60+ data points | Low | Low | Use Recharts optimization, memoization |
| Dark mode colors unreadable | Low | Low | Test thoroughly, use high-contrast colors |

## Questions & Decisions

### Q: Should we show all sensors or aggregate by type?
**Decision**: Show individual sensors initially. If >5 sensors, show top 5 by current temperature.

### Q: What temperature range for Y-axis?
**Decision**: Auto-scale based on data, with minimum range 0-60°C for readability.

### Q: Should we add temperature alerts in the chart?
**Decision**: Phase 1 - just show data. Phase 2 (future) - add threshold lines and visual alerts.

### Q: Historical data - how far back?
**Decision**: Use existing pattern - last 60 records (2 minutes at 2s interval). Consistent with CPU/Memory charts.

## Answer to User's Question

**"Precisará salvar informações no BD?"** (Will it need to save information to the DB?)

**Answer**: ❌ **NO** - No database changes required. Temperature data is already being collected every 2 seconds and saved to the database as JSON arrays in the `metrics` table. The `/api/v1/metrics/history` endpoint already returns this data, and the WebSocket already streams it in real-time. We only need to create a frontend component to visualize the existing data.

---

**Plan Status**: Ready for implementation
**Approval Required**: Yes
**Estimated Duration**: ~1 hour
**Risk Level**: Low
**Dependencies**: None (all data already available)
