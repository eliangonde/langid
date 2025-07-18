import struct

with open("model.bin", "wb") as f:
    f.write(struct.pack("II", *nb_ptc.shape))  # rows, cols
    f.write(nb_ptc.tobytes(order="C"))

    f.write(struct.pack("I", nb_pc.shape[0]))
    f.write(nb_pc.tobytes(order="C"))

    f.write(struct.pack("I", tk_nextmove_np.shape[0]))
    f.write(tk_nextmove_np.tobytes(order="C"))

    f.write(struct.pack("I", len(nb_classes)))
    for s in nb_classes:
        encoded = s.encode("utf-8")
        f.write(struct.pack("I", len(encoded)))
        f.write(encoded)

    f.write(struct.pack("I", len(tk_output)))
    for key, val in tk_output.items():
        f.write(struct.pack("I", key))
        f.write(struct.pack("I", len(val)))
        f.write(struct.pack(f"{len(val)}i", *val))
