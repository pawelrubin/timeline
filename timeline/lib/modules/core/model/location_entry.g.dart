// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'location_entry.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class LocationEntryAdapter extends TypeAdapter<LocationEntry> {
  @override
  final int typeId = 0;

  @override
  LocationEntry read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return LocationEntry(
      longitude: fields[1] as double,
      latitude: fields[0] as double,
      timestamp: fields[2] as DateTime,
      accuracy: fields[4] as double,
      activity: fields[3] as String,
    );
  }

  @override
  void write(BinaryWriter writer, LocationEntry obj) {
    writer
      ..writeByte(5)
      ..writeByte(0)
      ..write(obj.latitude)
      ..writeByte(1)
      ..write(obj.longitude)
      ..writeByte(2)
      ..write(obj.timestamp)
      ..writeByte(3)
      ..write(obj.activity)
      ..writeByte(4)
      ..write(obj.accuracy);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LocationEntryAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
