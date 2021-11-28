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
      altitude: fields[2] as double,
      latitude: fields[0] as double,
      dateTime: fields[3] as DateTime,
    );
  }

  @override
  void write(BinaryWriter writer, LocationEntry obj) {
    writer
      ..writeByte(4)
      ..writeByte(0)
      ..write(obj.latitude)
      ..writeByte(1)
      ..write(obj.longitude)
      ..writeByte(2)
      ..write(obj.altitude)
      ..writeByte(3)
      ..write(obj.dateTime);
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
