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
    return LocationEntry();
  }

  @override
  void write(BinaryWriter writer, LocationEntry obj) {
    writer..writeByte(0);
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
