"use client";

import { useState } from "react";
import { CameraTable } from "@/components/cameras/camera-table";
import { CameraForm } from "@/components/cameras/camera-form";
import { Dialog, DialogContent } from "@/components/ui/dialog";

export default function CamerasPage() {
  const [showForm, setShowForm] = useState(false);
  const [editId, setEditId] = useState<string | null>(null);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Cameras</h1>
        <p className="text-muted-foreground">Manage your camera fleet</p>
      </div>

      <CameraTable
        onCreate={() => setShowForm(true)}
        onEdit={(id) => setEditId(id)}
      />

      <Dialog open={showForm} onOpenChange={setShowForm}>
        <DialogContent className="max-w-lg">
          <CameraForm onSuccess={() => setShowForm(false)} onCancel={() => setShowForm(false)} />
        </DialogContent>
      </Dialog>

      <Dialog open={!!editId} onOpenChange={(o) => { if (!o) setEditId(null); }}>
        <DialogContent className="max-w-lg">
          {editId && (
            <CameraForm
              cameraId={editId}
              onSuccess={() => setEditId(null)}
              onCancel={() => setEditId(null)}
            />
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
